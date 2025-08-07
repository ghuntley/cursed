vibez.spill("Test start")

collab Drawable {
    slay draw()
}

squad Circle {
    slay draw() {
        vibez.spill("Drawing a circle")
    }
}

sus obj Drawable = Circle{}
vibez.spill("Created object:", obj)
obj.draw()

vibez.spill("Test end")
