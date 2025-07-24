-- version: 0.1.0
local api = BIN2LANG
local output = {}
table.insert(output, string.format("// Generated from '%s' by bin2lang\n", api.input_file))
local array_type = api.array_type or "const unsigned char"
table.insert(output, string.format("%s %s[%d] = {", array_type, api.array_name, #api.data))
local line_length = api.line_length or 16
local indent = string.rep(" ", api.indent or 4)
local line = {}
for i, byte in ipairs(api.data) do
    table.insert(line, string.format("0x%02X,", byte))
    if #line >= line_length or i == #api.data then
        table.insert(output, indent .. table.concat(line, " "))
        line = {}
    end
end
table.insert(output, "};")
return table.concat(output, "\n")
