from pointClass import *
if __name__=='__main':
	a=Point(2,2)
	b=Point(2,8)
	dist=a.distance(b)
	print(f"{dist}")
	a.move(2,2)
	print(a.x,a.y)