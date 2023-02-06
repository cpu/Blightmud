-- WIP spellcheck module.
local red_on = "\x1b[31m";
local ansi_off = "\x1b[0m";

prompt.add_prompt_listener(function (buf)
    local words = {}
    local word_start = 1
    for i = 1, #buf do
        local c = buf:sub(i, i)
        local stop = c:find("[%s%p]")

        if stop ~= nil then
            local word = buf:sub(word_start, i - 1)
            words[word_start] = word
            word_start = i + 1
        elseif i == #buf then
            local word = buf:sub(word_start, i)
            words[word_start] = word
        end
    end

    blight.output(string.format("\nbulding mask for '%s' (len: %d)\n", buf, #buf))
    local mask = {}
    for idx, word in pairs(words) do
        local even = #word % 2 == 0
        local off_idx = idx + #word
        blight.output(string.format("start: %d word: '%s' len: %d even: %s off_idx: %d\n",
                idx, word, #word, tostring(even), off_idx))
        if even then
            mask[idx] = red_on
            mask[off_idx] = ansi_off
        end
    end

    local masked = prompt_mask.set(buf, mask)
    blight.output(string.format("mask add: %s\n", masked))
end)
