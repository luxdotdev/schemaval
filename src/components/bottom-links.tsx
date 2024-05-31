"use client";

import { open } from "@tauri-apps/api/shell";

export function BottomLinks() {
  return (
    <div className="text-xs float-right text-muted-foreground">
      <button
        onClick={() => {
          open("https://lux.dev");
        }}
      >
        lux.dev
      </button>
      <span> &bull; </span>
      <button
        onClick={() => {
          open(
            "https://docs.google.com/spreadsheets/u/0/d/e/2PACX-1vTVvo1snE7irK2NLtWe3Xjk_1fGt97DGQNRyERVeai-6SeeeCPYw8fSRAa6vatwae-mQ0WDPQuq-q2-/pubhtml"
          );
        }}
      >
        view the schema
      </button>
    </div>
  );
}
