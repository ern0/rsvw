#!/usr/bin/env python3

# Example is from https://github.com/Stenway/RSV-Specification

def encode_rsv(rows):
	parts = []
	for row in rows:
		for value in row:
			if value is None: parts.append(b"\xFE")
			elif len(str(value)) > 0: parts.append(str(value).encode())
			parts.append(b"\xFF")
		parts.append(b"\xFD")
	return b"".join(parts)

def save_rsv(rows, file_path: str):
	with open(file_path, "wb") as file:
		file.write(encode_rsv(rows))

rows = [
	["Hello", "🌎"],
	[],
	[None, ""],
	[1,2,3,4],
	["","","a"],
	[None, "", None],
	[None],
	["", None],
	[""],
	["", None, "", ""],
]
save_rsv(rows, "example.rsv")
