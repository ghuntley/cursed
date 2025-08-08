# Variable lookup optimization test
yeet "mathz"
yeet "stringz" 
yeet "arrayz"

# Create many variables to test lookup performance
sus var1 drip = 10
sus var2 drip = 20
sus var3 drip = 30
sus var4 drip = 40
sus var5 drip = 50
sus var6 drip = 60
sus var7 drip = 70
sus var8 drip = 80
sus var9 drip = 90
sus var10 drip = 100

sus data []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
sus result drip = 0
sus counter drip = 0

# Heavy variable access pattern
bestie (counter < 200) {
    result = result + var1 + var2 + var3 + var4 + var5
    result = result + var6 + var7 + var8 + var9 + var10
    
    sus index drip = counter % len(data)
    result = result + data[index]
    
    counter = counter + 1
}

vibez.spill("Variable lookup result:", result)
