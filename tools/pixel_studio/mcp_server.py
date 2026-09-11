#!/usr/bin/env python3
"""
LandCraft Pixel Studio - MCP Server
Implements Model Context Protocol (JSON-RPC 2.0 over stdio) using standard library.
"""

import sys
import json
import os
import struct
import zlib
from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent.parent.parent
ASSETS_DIR = BASE_DIR / "assets"
BLOCK_DIR = ASSETS_DIR / "block"
PLAYER_DIR = ASSETS_DIR / "player"


def make_png(width: int, height: int, rgba_bytes: bytes) -> bytes:
    """Generates standard uncompressed/deflate PNG binary bytes."""
    sig = b"\x89PNG\r\n\x1a\n"
    ihdr_data = struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0)
    ihdr_crc = zlib.crc32(b"IHDR" + ihdr_data)
    ihdr = struct.pack(">I", len(ihdr_data)) + b"IHDR" + ihdr_data + struct.pack(">I", ihdr_crc)

    scanlines = bytearray()
    row_bytes = width * 4
    for y in range(height):
        scanlines.append(0)  # Filter type 0 (None)
        scanlines.extend(rgba_bytes[y * row_bytes : (y + 1) * row_bytes])

    compressed = zlib.compress(bytes(scanlines))
    idat_crc = zlib.crc32(b"IDAT" + compressed)
    idat = struct.pack(">I", len(compressed)) + b"IDAT" + compressed + struct.pack(">I", idat_crc)

    iend_crc = zlib.crc32(b"IEND")
    iend = struct.pack(">I", 0) + b"IEND" + struct.pack(">I", iend_crc)
    return sig + ihdr + idat + iend


def hex_to_rgba(hex_str: str) -> tuple[int, int, int, int]:
    hex_str = hex_str.lstrip("#")
    if len(hex_str) == 6:
        r = int(hex_str[0:2], 16)
        g = int(hex_str[2:4], 16)
        b = int(hex_str[4:6], 16)
        return (r, g, b, 255)
    elif len(hex_str) == 8:
        r = int(hex_str[0:2], 16)
        g = int(hex_str[2:4], 16)
        b = int(hex_str[4:6], 16)
        a = int(hex_str[6:8], 16)
        return (r, g, b, a)
    return (0, 0, 0, 0)


def create_block_texture(name: str, palette_matrix: list[list[str]]) -> str:
    """Creates a 16x16 block texture PNG and writes it to assets/block/."""
    BLOCK_DIR.mkdir(parents=True, exist_ok=True)
    target_path = BLOCK_DIR / f"{name}.png"

    rgba_bytes = bytearray()
    for y in range(16):
        row = palette_matrix[y] if y < len(palette_matrix) else ["#00000000"] * 16
        for x in range(16):
            color = row[x] if x < len(row) else "#00000000"
            r, g, b, a = hex_to_rgba(color)
            rgba_bytes.extend([r, g, b, a])

    png_data = make_png(16, 16, bytes(rgba_bytes))
    target_path.write_bytes(png_data)
    return f"Successfully generated 16x16 block texture: {target_path}"


def list_assets() -> dict:
    """Lists existing assets in the LandCraft project."""
    assets = {"blocks": [], "ui": [], "player": []}
    if BLOCK_DIR.exists():
        assets["blocks"] = [f.name for f in BLOCK_DIR.glob("*.png")]
    ui_dir = ASSETS_DIR / "ui"
    if ui_dir.exists():
        assets["ui"] = [f.name for f in ui_dir.glob("*.png")]
    if PLAYER_DIR.exists():
        assets["player"] = [f.name for f in PLAYER_DIR.glob("*") if f.is_file()]
    return assets


TOOLS = [
    {
        "name": "create_block_texture",
        "description": "Generate a 16x16 voxel block PNG texture and save directly to LandCraft/assets/block/.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "Name of the block (e.g. 'sand', 'dirt', 'wood')"
                },
                "palette_matrix": {
                    "type": "array",
                    "description": "16x16 2D array of hex color strings (e.g. '#866043')",
                    "items": {
                        "type": "array",
                        "items": {"type": "string"}
                    }
                }
            },
            "required": ["name", "palette_matrix"]
        }
    },
    {
        "name": "list_assets",
        "description": "List all texture, model, and UI assets available in LandCraft.",
        "inputSchema": {
            "type": "object",
            "properties": {}
        }
    }
]


def handle_request(req: dict) -> dict | None:
    method = req.get("method")
    req_id = req.get("id")

    if method == "initialize":
        return {
            "jsonrpc": "2.0",
            "id": req_id,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "landcraft-pixel-studio",
                    "version": "0.1.0"
                }
            }
        }
    elif method == "notifications/initialized":
        return None
    elif method == "tools/list":
        return {
            "jsonrpc": "2.0",
            "id": req_id,
            "result": {
                "tools": TOOLS
            }
        }
    elif method == "tools/call":
        params = req.get("params", {})
        tool_name = params.get("name")
        args = params.get("arguments", {})

        if tool_name == "create_block_texture":
            res = create_block_texture(args.get("name", "unnamed"), args.get("palette_matrix", []))
            return {
                "jsonrpc": "2.0",
                "id": req_id,
                "result": {
                    "content": [{"type": "text", "text": res}]
                }
            }
        elif tool_name == "list_assets":
            res = list_assets()
            return {
                "jsonrpc": "2.0",
                "id": req_id,
                "result": {
                    "content": [{"type": "text", "text": json.dumps(res, indent=2)}]
                }
            }
        else:
            return {
                "jsonrpc": "2.0",
                "id": req_id,
                "error": {"code": -32601, "message": f"Unknown tool: {tool_name}"}
            }

    return {
        "jsonrpc": "2.0",
        "id": req_id,
        "error": {"code": -32601, "message": f"Method not found: {method}"}
    }


def main():
    while True:
        try:
            line = sys.stdin.readline()
            if not line:
                break
            line = line.strip()
            if not line:
                continue
            req = json.loads(line)
            res = handle_request(req)
            if res is not None:
                sys.stdout.write(json.dumps(res) + "\n")
                sys.stdout.flush()
        except (KeyboardInterrupt, SystemExit):
            break
        except Exception as e:
            err_res = {
                "jsonrpc": "2.0",
                "id": None,
                "error": {"code": -32603, "message": str(e)}
            }
            sys.stdout.write(json.dumps(err_res) + "\n")
            sys.stdout.flush()


if __name__ == "__main__":
    main()
