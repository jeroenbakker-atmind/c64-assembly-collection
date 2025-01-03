t=0;s=math.sin;w=240
function TIC()
for i=0,32745 do
x=i%w
y=i/w
pix(x,y,(s((x+t)*.05)+s((y+t)*.07)+s((x+y+t)*.03))*5%16)
end
t=t+1
end
