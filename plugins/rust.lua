local api = BIN2LANG
local output = {}
table.insert(output, string.format("// Generated from '%s' by bin2lang\n", api.input_file))
table.insert(output, string.format("pub const %s: [u8; %d] = [", api.array_name:upper(), #api.data))
local line = {}
for i, byte in ipairs(api.data) do
    table.insert(line, string.format("0x%02X,", byte))
    if #line >= 12 or i == #api.data then
        table.insert(output, "    " .. table.concat(line, " "))
        line = {}
    end
end
table.insert(output, "];")
return table.concat(output, "\n")
