function init()
    print("Init from lua Basic.lua script");
    game:test()
    game = game:add(target(rand_range(game:width()),rand_range(game:height())))
    game = game:add(target(rand_range(game:width()),rand_range(game:height())))
    game = game:add(target(rand_range(game:width()),rand_range(game:height())))
end

function update()
    print("Update from lua Basic.lua script");
    --game = game:add(target(rand_range(game:width()),rand_range(game:height())))
end

function on_mouse_hit_target(index)
    print("Hit " .. index)
end 