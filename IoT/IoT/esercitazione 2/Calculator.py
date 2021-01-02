

	#la prima cosa da fare è definire la classe
	class Calculator:
				"""Calculator that has an attribute'brand and 4 method:with add(), sub(), mul(), div()"""
				def __init__(self, brand):     #self e poi quello che voglio usare
					self.brand=brand
				def add(self,a,b): #in questo modo ho aggiunto dei metodi 
					addition=a+b
					return addition
				def sub(self,a,b):
					return a-b
				def mult(self,a,b):
					return a*b
				def divide(self,a,b):
					if b!=0
						return a/b
					else:
						return None
				def showBrand(self):
					print(self.brand)

