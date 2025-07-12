# Edge cases for formatter testing
yeet"testz"
yeet"math"

# Tight spacing issues
slay tight_spacing(x normie)normie{
    sus result:=x+1*2-3/4
    sus array:=[1,2,3,4,5]
    sus tuple:=(x,result)
    nah result>0{
        result++
    }
    damn result
}

# Inconsistent braces
slay inconsistent_braces(x normie)normie{
    nah x>0{
        nah x<10{
            damn x*2
        }catch{
            damn x/2
        }
    }
    damn x
}

# Complex expressions with precedence
slay complex_precedence()normie{
    sus a:=1+2*3-4/2
    sus b:=a>=5&&a<=10
    sus c:=b||a==7
    damn nah c{1}catch{0}
}

# Multiple variable declarations
slay multiple_vars()lit{
    sus x,y,z normie=1,2,3
    sus(p,q,r):=(x+y,y+z,z+x)
    damn p+q+r>0
}
