function init()
    print("Init from lua Basic.lua script");
    game:test()
    game = game:add_target(target(100,100))
    game = game:add_target(target(200,200))
    game = game:add_target(target(300,300))
end

function update()
    print("Update from lua Basic.lua script");
    --game = game:add(target(rand_range(game:width()),rand_range(game:height())))
end

function on_mouse_hit_target(index)
    print("Hit " .. index)
   -- game = game:remove_target(index)
end 