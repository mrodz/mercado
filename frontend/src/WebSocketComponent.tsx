import { useEffect, useMemo, useState } from "react";

type ControlType = "add" | "remove" | "subscribe";

function parseSymbols(input: string): string[] {
  const out = input
    .split(/[\s,]+/g)
    .map(s => s.trim().toUpperCase())
    .filter(Boolean);

  return Array.from(new Set(out));
}

function WebSocketComponent() {
  const [messages, setMessages] = useState<string[]>([]);
  const [socket, setSocket] = useState<WebSocket | null>(null);

  const [symbolsInput, setSymbolsInput] = useState<string>("AAPL,MSFT");
  const [currentSymbols, setCurrentSymbols] = useState<string[]>([]);

  const parsedSymbols = useMemo(() => parseSymbols(symbolsInput), [symbolsInput]);

  useEffect(() => {
    const url = "https://127.0.0.1:8000/u/quotes/stream";
    const newSocket = new WebSocket(url);
    setSocket(newSocket);

    newSocket.onopen = () => {
      console.log("WebSocket connection established");
      setMessages(prev => [...prev, `[open] connected to ${url}`]);
    };

    newSocket.onmessage = (event) => {
      const data = typeof event.data === "string" ? event.data : String(event.data);
      setMessages(prev => [...prev, data]);
    };

    newSocket.onclose = () => {
      console.log("WebSocket connection closed");
      setMessages(prev => [...prev, "[close] disconnected"]);
    };

    newSocket.onerror = (error) => {
      console.error("WebSocket Error:", error);
      setMessages(prev => [...prev, "[error] websocket error"]);
    };

    return () => {
      newSocket.close();
    };
  }, []);

  function sendControl(type: ControlType, symbols: string[]) {
    if (!socket || socket.readyState !== WebSocket.OPEN) {
      setMessages(prev => [...prev, "[warn] socket not open"]);
      return;
    }

    const payload = JSON.stringify({ type, symbols });
    socket.send(payload);
    setMessages(prev => [...prev, `[sent] ${payload}`]);

    // Keep a local “current symbols” view (optional UI nicety)
    setCurrentSymbols(prev => {
      if (type === "subscribe") return symbols;
      if (type === "add") return Array.from(new Set([...prev, ...symbols]));
      // remove
      return prev.filter(s => !new Set(symbols).has(s));
    });
  }

  return (
    <div style={{ display: "grid", gap: 12, maxWidth: 800 }}>
      <div style={{ display: "flex", gap: 8, alignItems: "center", flexWrap: "wrap" }}>
        <input
          value={symbolsInput}
          onChange={(e) => setSymbolsInput(e.target.value)}
          placeholder="Symbols (comma/space separated): AAPL, MSFT, TSLA"
          style={{ flex: 1, minWidth: 320, padding: 8 }}
        />

        <button onClick={() => sendControl("add", parsedSymbols)} disabled={!parsedSymbols.length}>
          Add
        </button>

        <button onClick={() => sendControl("remove", parsedSymbols)} disabled={!parsedSymbols.length}>
          Remove
        </button>

        <button onClick={() => sendControl("subscribe", parsedSymbols)} disabled={!parsedSymbols.length}>
          Subscribe
        </button>
      </div>

      <div>
        <strong>Parsed symbols:</strong>{" "}
        {parsedSymbols.length ? parsedSymbols.join(", ") : "(none)"}
      </div>

      <div>
        <strong>Current symbols (local):</strong>{" "}
        {currentSymbols.length ? currentSymbols.join(", ") : "(none)"}
      </div>

      <div>
        <h3>Received Messages:</h3>
        <ul>
          {messages.map((msg, index) => (
            <li key={index}>
              <pre style={{ margin: 0, whiteSpace: "pre-wrap" }}>{msg}</pre>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}

export default WebSocketComponent;
