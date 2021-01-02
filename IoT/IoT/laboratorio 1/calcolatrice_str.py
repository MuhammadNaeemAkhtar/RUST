	#la prima cosa da fare è definire la classe
class Calculator():
	"""Calculator that has an attribute'brand and 4 method:with add(), sub(), mul(), div()"""
	def __init__(self,brand):
		self.brand = brand
	def add(self,a): #in questo modo ho aggiunto dei metodi 
		addition=sum(a)
		return addition
	def sub(self,a):
			for i in range (0, len(a)):
				if i==0:
					p=a[0]
				elif i>0:
					p=p-a[i]
			return p 
			

	def mult(self,a):
			for i in range (0, len(a)):
				if i==0:
					p=a[0] 
				elif i>0:
					p=p*a[i]
			return p 
			
	def divide(self,a):
		try: 
			for i in range (0, len(a)):
				if i==0:
					p=a[0]
				elif i>0:
					p=p/a[i]
			return p 
			
		except ZeroDivisionError:
			print("non è possibile")
			return None
	def showBrand(self):
		print(self.brand)

