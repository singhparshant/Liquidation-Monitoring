"use client";

import { useEffect, useMemo, useRef, useState } from "react";
import useWebSocket from "react-use-websocket";
import { WebSocketLike } from "react-use-websocket/dist/lib/types";
import {
  flexRender,
  getCoreRowModel,
  getPaginationRowModel,
  useReactTable,
  type ColumnDef,
  type PaginationState,
} from "@tanstack/react-table";


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

type RowData = { id: string; msg: BinanceMessage };

export default function Home() {
  
  const wsRef = useRef<WebSocketLike | null>(null);
  const msgSeqRef = useRef(0);
  const [messages, setMessages] = useState<RowData[]>([]);
  const [pagination, setPagination] = useState<PaginationState>({
    pageIndex: 0,
    pageSize: 20,
  });

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

  const columns = useMemo<Array<ColumnDef<RowData>>>(() => {
    return [
      {
        id: "time",
        header: "Time",
        cell: ({ row }) => (
          <span className="font-mono text-xs text-zinc-600 dark:text-zinc-400">
            {formatTime(row.original.msg.o.T)}
          </span>
        ),
      },
      {
        id: "symbol",
        header: "Symbol",
        cell: ({ row }) => (
          <span className="font-semibold">{row.original.msg.o.s}</span>
        ),
      },
      {
        id: "side",
        header: "Side",
        cell: ({ row }) => (
          <span
            className={[
              "inline-flex items-center rounded-full px-2 py-0.5 text-xs font-semibold ring-1 ring-inset",
              sideBadgeClass(row.original.msg.o.S),
            ].join(" ")}
          >
            {row.original.msg.o.S}
          </span>
        ),
      },
      { id: "type", header: "Type", cell: ({ row }) => row.original.msg.o.o },
      {
        id: "tif",
        header: "TIF",
        cell: ({ row }) => (
          <span className="font-mono text-xs">{row.original.msg.o.f}</span>
        ),
      },
      {
        id: "qty",
        header: () => <span className="block text-right">Qty</span>,
        cell: ({ row }) => (
          <span className="block text-right font-mono">
            {row.original.msg.o.q}
          </span>
        ),
      },
      {
        id: "price",
        header: () => <span className="block text-right">Price</span>,
        cell: ({ row }) => (
          <span className="block text-right font-mono">
            {row.original.msg.o.p}
          </span>
        ),
      },
      {
        id: "avg",
        header: () => <span className="block text-right">Avg</span>,
        cell: ({ row }) => (
          <span className="block text-right font-mono">
            {row.original.msg.o.ap}
          </span>
        ),
      },
      {
        id: "status",
        header: "Status",
        cell: ({ row }) => (
          <span
            className={[
              "inline-flex items-center rounded-full px-2 py-0.5 text-xs font-semibold ring-1 ring-inset",
              statusBadgeClass(row.original.msg.o.X),
            ].join(" ")}
          >
            {row.original.msg.o.X}
          </span>
        ),
      },
      {
        id: "last",
        header: () => <span className="block text-right">Last</span>,
        cell: ({ row }) => (
          <span className="block text-right font-mono">
            {row.original.msg.o.l}
          </span>
        ),
      },
      {
        id: "filled",
        header: () => <span className="block text-right">Filled</span>,
        cell: ({ row }) => (
          <span className="block text-right font-mono">
            {row.original.msg.o.z}
          </span>
        ),
      },
    ];
  }, []);

  const table = useReactTable({
    data: messages,
    columns,
    state: { pagination },
    onPaginationChange: setPagination,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
  });
  
  return (
    <div className="flex min-h-screen items-center justify-center bg-zinc-50 font-sans dark:bg-black">
      <main className="flex min-h-screen w-full max-w-6xl flex-col gap-6 py-16 px-4 bg-white dark:bg-black sm:px-8">
        <div className="flex items-center justify-between gap-4">
          <div className="flex items-center gap-3">
            <div className="text-sm text-zinc-600 dark:text-zinc-300 inline-flex items-center gap-2">
              <span
                className={[
                  "h-2.5 w-2.5 rounded-full ring-2 ring-inset animate-pulse",
                  readyState === 1
                    ? "bg-emerald-500 ring-emerald-200 dark:ring-emerald-900"
                    : "bg-rose-500 ring-rose-200 dark:ring-rose-900",
                ].join(" ")}
                aria-label={readyState === 1 ? "WebSocket connected" : "WebSocket disconnected"}
                title={readyState === 1 ? "connected" : "disconnected"}
              />
              WS:{" "}
              <span className="font-mono">
                {readyState === 1 ? "Connected" : `Disconnected`}
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
                {table.getHeaderGroups().map((headerGroup) =>
                  headerGroup.headers.map((header) => (
                    <th key={header.id}>
                      {header.isPlaceholder
                        ? null
                        : flexRender(
                            header.column.columnDef.header,
                            header.getContext()
                          )}
                    </th>
                  ))
                )}
              </tr>
            </thead>
            <tbody className="divide-y divide-zinc-100 dark:divide-zinc-900">
              {table.getRowModel().rows.map((row) => (
                <tr
                  key={row.original.id}
                  className="odd:bg-white even:bg-zinc-50 hover:bg-zinc-100 dark:odd:bg-black dark:even:bg-zinc-950 dark:hover:bg-zinc-900"
                >
                  {row.getVisibleCells().map((cell) => (
                    <td key={cell.id} className="px-3 py-2">
                      {flexRender(cell.column.columnDef.cell, cell.getContext())}
                    </td>
                  ))}
                </tr>
              ))}
            </tbody>
          </table>
        </div>

        <div className="flex items-center justify-between gap-4">
          <div className="text-xs text-zinc-500 dark:text-zinc-400">
            Page{" "}
            <span className="font-mono">
              {table.getState().pagination.pageIndex + 1}
            </span>{" "}
            of{" "}
            <span className="font-mono">{table.getPageCount()}</span>
          </div>
          <div className="flex items-center gap-2">
            <button
              className="rounded-md border border-zinc-200 px-3 py-1.5 text-sm text-zinc-700 disabled:opacity-50 dark:border-zinc-800 dark:text-zinc-200"
              onClick={() => table.previousPage()}
              disabled={!table.getCanPreviousPage()}
            >
              Prev
            </button>
            <button
              className="rounded-md border border-zinc-200 px-3 py-1.5 text-sm text-zinc-700 disabled:opacity-50 dark:border-zinc-800 dark:text-zinc-200"
              onClick={() => table.nextPage()}
              disabled={!table.getCanNextPage()}
            >
              Next
            </button>
          </div>
        </div>
      </main>
    </div>
  );
}
