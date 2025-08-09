sus data []drip = [10, 20, 30]
sus index drip = 0

vibez.spill("Before loop - data:", data)
vibez.spill("Before loop - index:", index)
vibez.spill("Before loop - data[0]:", data[0])

bestie (index < 1) {
    vibez.spill("In loop - index:", index)
    vibez.spill("In loop - data:", data)
    vibez.spill("In loop - data[index]:", data[index])
    index = index + 1
}
