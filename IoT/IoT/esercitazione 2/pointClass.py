class Point(object):
	def __init__(self, coordX, coordY):
		self.x=coordX
		self.y=coordY
	def distance(self, otherPoint):
		return math.sqrt((self.x-otherPoint.x)**2+(self.y-otherPoint.y)**2)
	def move(self, dX, dY):
		self.x+=dX
		self.y+=dY
