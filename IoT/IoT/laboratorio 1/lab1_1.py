from calcolatrice_str import *
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
			x= (input("inserisci una stringa di numeri separati da virgola"))
			numeri=x.split(',')
			for i in range (len(numeri)):
				numeri[i]=float(numeri[i])

			addition= calc.add(numeri)
			somma= {"num1":x,"risultato":addition}
			addizione=json.dumps(somma)
			print(addizione)

		elif digit== "sub":
			x= (input("inserisci una stringa di numeri separati da virgola"))
			numeri=x.split(',')
			for i in range (len(numeri)):
				numeri[i]=float(numeri[i])

			sottrazione= calc.sub(numeri)
			differenza={"num1":x, "risultato":sottrazione}
			sottrazion=json.dumps(differenza)
			print(sottrazion)

		elif digit== "mul":
			x= (input("inserisci una stringa di numeri separati da virgola"))
			numeri=x.split(',')
			for i in range (len(numeri)):
				numeri[i]=float(numeri[i])


			moltiplicazione= calc.mult(numeri)
			mol={"num1":x, "risultato":moltiplicazione}
			multiplication= json.dumps(mol)
			print(multiplication)

		elif digit== "div":
			x= (input("inserisci una stringa di numeri separati da virgola"))
			numeri=x.split(',')
			for i in range (len(numeri)):
				numeri[i]=float(numeri[i])

			divisione= calc.divide(numeri)
			division={"num2":x, "risultato":divisione}
			divis= json.dumps(division)
			print(divis)
		elif digit== "exit":
			break
