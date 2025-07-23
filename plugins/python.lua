local api = BIN2LANG
local output = {}
table.insert(output, string.format("# Generated from '%s' by bin2lang", api.input_file))
table.insert(output, string.format("%s = b'\\", api.array_name))
local line = ""
for _, byte in ipairs(api.data) do
    line = line .. string.format("\\x%02x", byte)
    if #line >= 80 then -- Approx 20 bytes per line
        table.insert(output, "    " .. line)
        line = ""
    end
end
if #line > 0 then table.insert(output, "    " .. line) end
table.insert(output, "'")
return table.concat(output, "\\\n")
