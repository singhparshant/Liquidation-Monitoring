"use client";

import Image from "next/image";
import { useEffect, useRef, useState } from "react";
import useWebSocket from "react-use-websocket";
import { WebSocketLike } from "react-use-websocket/dist/lib/types";


interface BinanceOrder {
  s: string,  // Symbol
  S: string,  // Side
  o: string,  // Order Type
  f: string,  // Time in Force
  q: string,  // Original Quantity
  p: string,  // Price
  ap: string, // Average Price
  X: string,  // Order Status
  l: string,  // Order Last Filled Quantity
  z: string,  // Order Filled Accumulated Quantity
  T: number,     // Order Trade Time
}

interface BinanceMessage {
  e: string;
  E: number;
  o: BinanceOrder;
}

export default function Home() {
  
  const wsRef = useRef<WebSocketLike | null>(null);
  const msgSeqRef = useRef(0);
  const [messages, setMessages] = useState<Array<{ id: string; msg: BinanceMessage }>>([]);

  const formatTime = (ms: number) => new Date(ms).toLocaleString();
  const sideBadgeClass = (side: string) =>
    side === "BUY"
      ? "bg-emerald-100 text-emerald-800 ring-emerald-200 dark:bg-emerald-950 dark:text-emerald-200 dark:ring-emerald-900"
      : "bg-rose-100 text-rose-800 ring-rose-200 dark:bg-rose-950 dark:text-rose-200 dark:ring-rose-900";
  const statusBadgeClass = (status: string) =>
    status === "FILLED"
      ? "bg-sky-100 text-sky-800 ring-sky-200 dark:bg-sky-950 dark:text-sky-200 dark:ring-sky-900"
      : "bg-zinc-100 text-zinc-800 ring-zinc-200 dark:bg-zinc-900 dark:text-zinc-200 dark:ring-zinc-800";

  const { getWebSocket, readyState } = useWebSocket("ws://localhost:3001/ws", {
    onOpen: () => {
      console.log('WebSocket connection established.');
      if (getWebSocket() !== null) {
        wsRef.current = getWebSocket();
      }
    },
    onMessage: (event) => {
      try {
        const parsed: BinanceMessage = JSON.parse(event.data);
        const id = `${parsed.o.T}-${msgSeqRef.current++}`;
        setMessages((prev) => [{ id, msg: parsed }, ...prev].slice(0, 200));
      } catch (e) {
        console.error("Failed to parse message:", e);
      }
    },
    onClose: () => {
      console.log('WebSocket connection closed.');
    }
  });

  // Lifecycle cleanup
  useEffect(() => {
    return () => {
      if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
        wsRef.current.close();
      }
    };
  }, []);
  
  return (
    <div className="flex min-h-screen items-center justify-center bg-zinc-50 font-sans dark:bg-black">
      <main className="flex min-h-screen w-full max-w-6xl flex-col gap-6 py-16 px-4 bg-white dark:bg-black sm:px-8">
        <div className="flex items-center justify-between gap-4">
          <div className="flex items-center gap-3">
            <Image
              className="dark:invert"
              src="/next.svg"
              alt="Next.js logo"
              width={96}
              height={20}
              priority
            />
            <div className="text-sm text-zinc-600 dark:text-zinc-300">
              WS:{" "}
              <span className="font-mono">
                {readyState === 1 ? "connected" : readyState}
              </span>
            </div>
          </div>
          <div className="text-xs text-zinc-500 dark:text-zinc-400">
            Showing {messages.length} events
          </div>
        </div>

        <div className="w-full overflow-x-auto rounded-xl border border-zinc-200 dark:border-zinc-800">
          <table className="min-w-[980px] w-full text-sm">
            <thead className="sticky top-0 bg-zinc-50 text-xs uppercase tracking-wide text-zinc-600 dark:bg-zinc-950 dark:text-zinc-400">
              <tr className="[&>th]:px-3 [&>th]:py-3 [&>th]:text-left">
                <th>Time</th>
                <th>Symbol</th>
                <th>Side</th>
                <th>Type</th>
                <th>TIF</th>
                <th className="text-right">Qty</th>
                <th className="text-right">Price</th>
                <th className="text-right">Avg</th>
                <th>Status</th>
                <th className="text-right">Last</th>
                <th className="text-right">Filled</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-zinc-100 dark:divide-zinc-900">
              {messages.map(({ id, msg: m }) => (
                <tr
                  key={id}
                  className="odd:bg-white even:bg-zinc-50 hover:bg-zinc-100 dark:odd:bg-black dark:even:bg-zinc-950 dark:hover:bg-zinc-900"
                >
                  <td className="px-3 py-2 font-mono text-xs text-zinc-600 dark:text-zinc-400">
                    {formatTime(m.o.T)}
                  </td>
                  <td className="px-3 py-2 font-semibold">{m.o.s}</td>
                  <td className="px-3 py-2">
                    <span
                      className={[
                        "inline-flex items-center rounded-full px-2 py-0.5 text-xs font-semibold ring-1 ring-inset",
                        sideBadgeClass(m.o.S),
                      ].join(" ")}
                    >
                      {m.o.S}
                    </span>
                  </td>
                  <td className="px-3 py-2">{m.o.o}</td>
                  <td className="px-3 py-2 font-mono text-xs">{m.o.f}</td>
                  <td className="px-3 py-2 text-right font-mono">{m.o.q}</td>
                  <td className="px-3 py-2 text-right font-mono">{m.o.p}</td>
                  <td className="px-3 py-2 text-right font-mono">{m.o.ap}</td>
                  <td className="px-3 py-2">
                    <span
                      className={[
                        "inline-flex items-center rounded-full px-2 py-0.5 text-xs font-semibold ring-1 ring-inset",
                        statusBadgeClass(m.o.X),
                      ].join(" ")}
                    >
                      {m.o.X}
                    </span>
                  </td>
                  <td className="px-3 py-2 text-right font-mono">{m.o.l}</td>
                  <td className="px-3 py-2 text-right font-mono">{m.o.z}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </main>
    </div>
  );
}
