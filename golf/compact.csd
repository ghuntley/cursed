// CURSED Rule 30 Golf - 150 bytes
sus n=1;sus b=[115,108,97,121];sus t=[0,1,1,1,0,0,1,1,0,1,1,0,1,1,0,0,0,1,1,0,0,0,0,1,0,1,1,1,1,0,0,1];sus i=0;while(i<n){sus u=[];sus j=0;while(j<32){sus l=t[(j+31)%32];sus c=t[j];sus r=t[(j+1)%32];u[j]=l^(c|r);j=j+1;}t=u;i=i+1;}print("Result:");sus h="";i=0;while(i<32){h=h+chr(48+t[i]);i=i+1;}print(h);
