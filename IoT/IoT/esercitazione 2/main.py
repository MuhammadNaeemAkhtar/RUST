from calcolatrice import *
import json
if __name__ == '__main__':
	calc= Calculator("casio")
	while True:
		digit=input(""" 
			digita add se vuoi fare addizione
			digita sub se vuoi fare sottrazione
			digita mul se vuoi fare moltiplicazione
			digita div se vuoi fare divisione
			digita exit per uscire
			""")
		if digit== "add":
			x= float(input("inserisci un numero"))
			y=float(input("inserisci un numero"))
			addition= calc.add(x,y)
			somma= {"num1":x, "num2":y, "risultato":addition}
			addizione=json.dumps(somma)
			print(addizione)
		elif digit== "sub":
			x= float(input("inserisci un numero"))
			y=float(input("inserisci un numero"))
			sottrazione= calc.sub(x,y)
			differenza={"num1":x, "num2":y, "risultato":sottrazione}
			sottrazion=json.dumps(differenza)
			print(sottrazion)
		elif digit== "mul":
			x= float(input("inserisci un numero"))
			y=float(input("inserisci un numero"))
			moltiplicazione= calc.mult(x,y)
			mol={"num1":x, "num2":y, "risultato":moltiplicazione}
			multiplication= json.dumps(mol)
			print(multiplication)
		elif digit== "div":
			x= float(input("inserisci un numero"))
			y=float(input("inserisci un numero"))
			divisione= calc.div(x,y)
			division={"num2":x, "num2":y, "risultato":divisione}
			divis= json.dumps(division)
			print(divis)
		elif digit== "exit":
			break
