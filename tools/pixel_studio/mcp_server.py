#!/usr/bin/env python3
"""
LandCraft Pixel Studio - MCP Server
Full-featured pixel-art creation server implementing Model Context Protocol (JSON-RPC 2.0 over stdio).
Zero external dependencies (uses Python standard library: struct, zlib, json, sys).
"""

import sys
import json
import struct
import zlib
from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent.parent.parent
ASSETS_DIR = BASE_DIR / "assets"
BLOCK_DIR = ASSETS_DIR / "block"
PLAYER_DIR = ASSETS_DIR / "player"
UI_DIR = ASSETS_DIR / "ui"

# Active canvas state in memory
CANVAS = {
    "name": "untitled",
    "width": 16,
    "height": 16,
    "pixels": [["#00000000" for _ in range(16)] for _ in range(16)]
}


def make_png(width: int, height: int, rgba_bytes: bytes) -> bytes:
    """Encodes raw RGBA bytes into standard spec-compliant PNG binary data."""
    sig = b"\x89PNG\r\n\x1a\n"
    ihdr_data = struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0)
    ihdr = struct.pack(">I", len(ihdr_data)) + b"IHDR" + ihdr_data + struct.pack(">I", zlib.crc32(b"IHDR" + ihdr_data))

    scanlines = bytearray()
    row_bytes = width * 4
    for y in range(height):
        scanlines.append(0)  # Filter type 0 (None)
        scanlines.extend(rgba_bytes[y * row_bytes : (y + 1) * row_bytes])

    compressed = zlib.compress(bytes(scanlines))
    idat = struct.pack(">I", len(compressed)) + b"IDAT" + compressed + struct.pack(">I", zlib.crc32(b"IDAT" + compressed))

    iend = struct.pack(">I", 0) + b"IEND" + struct.pack(">I", zlib.crc32(b"IEND"))
    return sig + ihdr + idat + iend


def parse_png(data: bytes) -> tuple[int, int, list[list[str]]]:
    """Decodes standard PNG binary data into width, height, and hex pixel grid."""
    if not data.startswith(b"\x89PNG\r\n\x1a\n"):
        raise ValueError("Invalid PNG signature")

    pos = 8
    width, height = 0, 0
    idat_chunks = bytearray()

    while pos < len(data):
        chunk_len = struct.unpack(">I", data[pos : pos + 4])[0]
        chunk_type = data[pos + 4 : pos + 8]
        chunk_data = data[pos + 8 : pos + 8 + chunk_len]
        pos += 12 + chunk_len

        if chunk_type == b"IHDR":
            width, height = struct.unpack(">II", chunk_data[:8])
        elif chunk_type == b"IDAT":
            idat_chunks.extend(chunk_data)
        elif chunk_type == b"IEND":
            break

    raw_decompressed = zlib.decompress(bytes(idat_chunks))
    pixels = [["#00000000" for _ in range(width)] for _ in range(height)]

    row_len = width * 4
    for y in range(height):
        offset = y * (row_len + 1) + 1
        row = raw_decompressed[offset : offset + row_len]
        for x in range(width):
            r = row[x * 4]
            g = row[x * 4 + 1]
            b = row[x * 4 + 2]
            a = row[x * 4 + 3]
            pixels[y][x] = f"#{r:02x}{g:02x}{b:02x}{a:02x}"

    return width, height, pixels


def hex_to_rgba(hex_str: str) -> tuple[int, int, int, int]:
    hex_str = hex_str.lstrip("#")
    if len(hex_str) == 6:
        return int(hex_str[0:2], 16), int(hex_str[2:4], 16), int(hex_str[4:6], 16), 255
    elif len(hex_str) == 8:
        return int(hex_str[0:2], 16), int(hex_str[2:4], 16), int(hex_str[4:6], 16), int(hex_str[6:8], 16)
    return 0, 0, 0, 0


# --- Drawing Primitives ---

def tool_create_canvas(name: str, width: int = 16, height: int = 16, background: str = "#00000000") -> str:
    global CANVAS
    width = max(1, min(128, width))
    height = max(1, min(128, height))
    CANVAS = {
        "name": name,
        "width": width,
        "height": height,
        "pixels": [[background for _ in range(width)] for _ in range(height)]
    }
    return f"Created canvas '{name}' ({width}x{height}) with background {background}"


def tool_draw_pixels(points: list[dict]) -> str:
    w, h = CANVAS["width"], CANVAS["height"]
    count = 0
    for pt in points:
        x, y, color = pt.get("x", 0), pt.get("y", 0), pt.get("color", "#ffffffff")
        if 0 <= x < w and 0 <= y < h:
            CANVAS["pixels"][y][x] = color
            count += 1
    return f"Plotted {count} pixels on '{CANVAS['name']}'"


def tool_draw_line(x0: int, y0: int, x1: int, y1: int, color: str) -> str:
    """Bresenham's line algorithm."""
    w, h = CANVAS["width"], CANVAS["height"]
    dx = abs(x1 - x0)
    dy = abs(y1 - y0)
    sx = 1 if x0 < x1 else -1
    sy = 1 if y0 < y1 else -1
    err = dx - dy
    plotted = 0

    while True:
        if 0 <= x0 < w and 0 <= y0 < h:
            CANVAS["pixels"][y0][x0] = color
            plotted += 1
        if x0 == x1 and y0 == y1:
            break
        e2 = 2 * err
        if e2 > -dy:
            err -= dy
            x0 += sx
        if e2 < dx:
            err += dx
            y0 += sy

    return f"Drew line from ({x0},{y0}) to ({x1},{y1}) [{plotted} pixels]"


def tool_draw_rectangle(x: int, y: int, width: int, height: int, color: str, fill: bool = True) -> str:
    w, h = CANVAS["width"], CANVAS["height"]
    plotted = 0
    for row in range(y, y + height):
        for col in range(x, x + width):
            if 0 <= col < w and 0 <= row < h:
                if fill or col == x or col == x + width - 1 or row == y or row == y + height - 1:
                    CANVAS["pixels"][row][col] = color
                    plotted += 1
    return f"Rendered {'filled' if fill else 'outlined'} rectangle at ({x},{y}) size ({width}x{height}) [{plotted} pixels]"


def tool_apply_dither(x: int, y: int, width: int, height: int, color1: str, color2: str) -> str:
    """Checkerboard 2x2 dithering pattern for realistic retro voxel gradients."""
    w, h = CANVAS["width"], CANVAS["height"]
    plotted = 0
    for row in range(y, y + height):
        for col in range(x, x + width):
            if 0 <= col < w and 0 <= row < h:
                CANVAS["pixels"][row][col] = color1 if (col + row) % 2 == 0 else color2
                plotted += 1
    return f"Applied 2x2 dithering pattern in rect ({x},{y}) size ({width}x{height}) [{plotted} pixels]"


def tool_fill_area(x: int, y: int, fill_color: str) -> str:
    w, h = CANVAS["width"], CANVAS["height"]
    if not (0 <= x < w and 0 <= y < h):
        return "Start point out of bounds"
    target_color = CANVAS["pixels"][y][x]
    if target_color == fill_color:
        return "Color match, nothing filled"

    stack = [(x, y)]
    filled = 0
    while stack:
        cx, cy = stack.pop()
        if not (0 <= cx < w and 0 <= cy < h):
            continue
        if CANVAS["pixels"][cy][cx] == target_color:
            CANVAS["pixels"][cy][cx] = fill_color
            filled += 1
            stack.extend([(cx + 1, cy), (cx - 1, cy), (cx, cy + 1), (cx, cy - 1)])

    return f"Flood-filled {filled} pixels with {fill_color}"


def tool_save_canvas(folder: str = "block", filename: str = "") -> str:
    if not filename:
        filename = f"{CANVAS['name']}.png"
    if not filename.endswith(".png"):
        filename += ".png"

    dest_folder = BLOCK_DIR if folder == "block" else (PLAYER_DIR if folder == "player" else UI_DIR)
    dest_folder.mkdir(parents=True, exist_ok=True)
    target_path = dest_folder / filename

    w, h = CANVAS["width"], CANVAS["height"]
    rgba_bytes = bytearray()
    for row in CANVAS["pixels"]:
        for color in row:
            rgba_bytes.extend(hex_to_rgba(color))

    target_path.write_bytes(make_png(w, h, bytes(rgba_bytes)))
    return f"Successfully saved {w}x{h} PNG to {target_path}"


def tool_load_canvas(folder: str = "block", filename: str = "") -> str:
    global CANVAS
    dest_folder = BLOCK_DIR if folder == "block" else (PLAYER_DIR if folder == "player" else UI_DIR)
    target_path = dest_folder / filename
    if not target_path.exists():
        return f"File not found: {target_path}"

    w, h, px = parse_png(target_path.read_bytes())
    CANVAS = {
        "name": target_path.stem,
        "width": w,
        "height": h,
        "pixels": px
    }
    return f"Loaded canvas '{target_path.stem}' ({w}x{h}) from {target_path}"


def tool_get_canvas_preview() -> str:
    """Returns an ASCII/color map of the current canvas state."""
    w, h = CANVAS["width"], CANVAS["height"]
    lines = [f"Canvas: {CANVAS['name']} ({w}x{h})"]
    for y in range(h):
        line = "".join("[]" if CANVAS["pixels"][y][x] not in ("#00000000", None) else ".." for x in range(w))
        lines.append(f"{y:02d} | {line}")
    return "\n".join(lines)


def tool_list_assets() -> dict:
    assets = {"blocks": [], "ui": [], "player": []}
    if BLOCK_DIR.exists():
        assets["blocks"] = [f.name for f in BLOCK_DIR.glob("*.png")]
    if UI_DIR.exists():
        assets["ui"] = [f.name for f in UI_DIR.glob("*.png")]
    if PLAYER_DIR.exists():
        assets["player"] = [f.name for f in PLAYER_DIR.glob("*") if f.is_file()]
    return assets


TOOLS = [
    {
        "name": "create_canvas",
        "description": "Initialize a new pixel-art canvas in memory (defaults to 16x16 for blocks).",
        "inputSchema": {
            "type": "object",
            "properties": {
                "name": {"type": "string", "description": "Asset name, e.g. 'obsidian'"},
                "width": {"type": "integer", "default": 16},
                "height": {"type": "integer", "default": 16},
                "background": {"type": "string", "default": "#00000000"}
            },
            "required": ["name"]
        }
    },
    {
        "name": "draw_pixels",
        "description": "Plot multiple pixels on the active canvas using coordinate-color pairs.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "points": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "x": {"type": "integer"},
                            "y": {"type": "integer"},
                            "color": {"type": "string"}
                        },
                        "required": ["x", "y", "color"]
                    }
                }
            },
            "required": ["points"]
        }
    },
    {
        "name": "draw_line",
        "description": "Draw a line between two coordinates using Bresenham's algorithm.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "x0": {"type": "integer"},
                "y0": {"type": "integer"},
                "x1": {"type": "integer"},
                "y1": {"type": "integer"},
                "color": {"type": "string"}
            },
            "required": ["x0", "y0", "x1", "y1", "color"]
        }
    },
    {
        "name": "draw_rectangle",
        "description": "Draw an outlined or filled rectangle on the active canvas.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "x": {"type": "integer"},
                "y": {"type": "integer"},
                "width": {"type": "integer"},
                "height": {"type": "integer"},
                "color": {"type": "string"},
                "fill": {"type": "boolean", "default": True}
            },
            "required": ["x", "y", "width", "height", "color"]
        }
    },
    {
        "name": "apply_dither",
        "description": "Apply a 2x2 retro checkerboard dither pattern between two colors.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "x": {"type": "integer"},
                "y": {"type": "integer"},
                "width": {"type": "integer"},
                "height": {"type": "integer"},
                "color1": {"type": "string"},
                "color2": {"type": "string"}
            },
            "required": ["x", "y", "width", "height", "color1", "color2"]
        }
    },
    {
        "name": "fill_area",
        "description": "Flood-fill an area of contiguous matching pixels.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "x": {"type": "integer"},
                "y": {"type": "integer"},
                "fill_color": {"type": "string"}
            },
            "required": ["x", "y", "fill_color"]
        }
    },
    {
        "name": "save_canvas",
        "description": "Export the active canvas directly to the game's assets/ directory.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "folder": {"type": "string", "enum": ["block", "player", "ui"], "default": "block"},
                "filename": {"type": "string", "description": "e.g. 'iron_ore.png'"}
            }
        }
    },
    {
        "name": "load_canvas",
        "description": "Load an existing PNG asset from assets/ into the canvas for programmatic editing.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "folder": {"type": "string", "enum": ["block", "player", "ui"], "default": "block"},
                "filename": {"type": "string", "description": "e.g. '200902092053_terrain.png'"}
            },
            "required": ["filename"]
        }
    },
    {
        "name": "get_canvas_preview",
        "description": "Inspect the current canvas layout in ASCII preview format.",
        "inputSchema": {
            "type": "object",
            "properties": {}
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
                "capabilities": {"tools": {}},
                "serverInfo": {
                    "name": "landcraft-pixel-studio",
                    "version": "0.2.0"
                }
            }
        }
    elif method == "notifications/initialized":
        return None
    elif method == "tools/list":
        return {
            "jsonrpc": "2.0",
            "id": req_id,
            "result": {"tools": TOOLS}
        }
    elif method == "tools/call":
        params = req.get("params", {})
        tool_name = params.get("name")
        args = params.get("arguments", {})

        try:
            if tool_name == "create_canvas":
                res = tool_create_canvas(args["name"], args.get("width", 16), args.get("height", 16), args.get("background", "#00000000"))
            elif tool_name == "draw_pixels":
                res = tool_draw_pixels(args.get("points", []))
            elif tool_name == "draw_line":
                res = tool_draw_line(args["x0"], args["y0"], args["x1"], args["y1"], args["color"])
            elif tool_name == "draw_rectangle":
                res = tool_draw_rectangle(args["x"], args["y"], args["width"], args["height"], args["color"], args.get("fill", True))
            elif tool_name == "apply_dither":
                res = tool_apply_dither(args["x"], args["y"], args["width"], args["height"], args["color1"], args["color2"])
            elif tool_name == "fill_area":
                res = tool_fill_area(args["x"], args["y"], args["fill_color"])
            elif tool_name == "save_canvas":
                res = tool_save_canvas(args.get("folder", "block"), args.get("filename", ""))
            elif tool_name == "load_canvas":
                res = tool_load_canvas(args.get("folder", "block"), args["filename"])
            elif tool_name == "get_canvas_preview":
                res = tool_get_canvas_preview()
            elif tool_name == "list_assets":
                res = json.dumps(tool_list_assets(), indent=2)
            else:
                return {
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "error": {"code": -32601, "message": f"Unknown tool: {tool_name}"}
                }

            return {
                "jsonrpc": "2.0",
                "id": req_id,
                "result": {
                    "content": [{"type": "text", "text": str(res)}]
                }
            }
        except Exception as e:
            return {
                "jsonrpc": "2.0",
                "id": req_id,
                "error": {"code": -32603, "message": f"Execution error: {str(e)}"}
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
