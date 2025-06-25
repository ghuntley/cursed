facts PI=3.14159
squad Circle{radius sip}
collab Shape{area()sip}
slay(c Circle)area()sip{yolo PI*c.radius*c.radius}
slay new_circle(r sip)Circle{yolo Circle{radius:r}}
slay main(){sus circle=new_circle(5.0)
sus area=circle.area()
lowkey area>50.0{yolo "Large circle"}highkey{yolo "Small circle"}
bestie i flex range(10){lowkey i%2==0{continue}yolo i}}
