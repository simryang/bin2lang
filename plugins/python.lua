local api = BIN2LANG
local output = {}
table.insert(output, string.format("# Generated from '%s' by bin2lang", api.input_file))
local python_type = api.python_type or "bytes"
table.insert(output, string.format("%s: %s = b'\\", api.array_name, python_type))
local line_length = api.line_length or 16
local indent = string.rep(" ", api.indent or 4)
local line = ""
local count = 0
for _, byte in ipairs(api.data) do
    line = line .. string.format("\\x%02x", byte)
    count = count + 1
    if count >= line_length then
        table.insert(output, indent .. line)
        line = ""
        count = 0
    end
end
if #line > 0 then table.insert(output, indent .. line) end
table.insert(output, "'")
return table.concat(output, "\\\n")
