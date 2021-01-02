
if __name__=="__main__":
	a=2
	b=3
	c=a+b
	print("%d" %c)
	print("%d-%d" %(b,a))
	print(f"{a}+{b}={c}")
	#print(f"{pi:.4}")
	g=("giovanni")
	print(f"my name is {g}")
	age=23
	date=19
	month=3
	year=1996
	stringa="19/03/1996"
	print(f"My name is {g} and i'm {age} years old, i was born the {date}/{month}/{year}")
	print(f"My name is {g} and i'm {age} years old, i was born the {stringa}")
	pi= 3.15169265
	print(f"{pi:.4}")

	print(f"2+3={2+3}")


	number=input("inserisci quanto è vecchio")
	altezza=input("inserisci quanto sei alto")
	print(f"il monte everest è vecchio {number} e alto {altezza} metri")

	f=open('myfile.txt','r')
	fileContent=f.read()
	print(f"{fileContent}")
	#f.close()

	f=open('myfile.txt','w')
	f.write('line to write') #questa riga sovrascrive il file 
	f.close()
	print(f"{f}")


	s=open("original.txt","r")
	fileContent=s.read()
	s.close()
	print(f"{fileContent}")

# devo copiare il contenuto del file originale in copy
	c=open("copy.txt","w")
	c.write(fileContent) # per copiare il contenuto di una variabile non devi inserire le virgolette
	c.close()
	print (f"{c}")

	cc=open("copy.txt", "a")
	cc.write("il contenuto originale e' :")
	cc.close()
	print(f"{cc}")


#esercizio 6 uso di if elif else
	numero=int(input("inserisci un numero\n")) #devo necessariamente mettere il comando int
	if numero % 3 == 0: #fai attenzione ai due punti dopo ogni richiesta
		print (f"il numero è divisibile per 3")
	elif numero % 2 == 0:
		print (f"il numero è divisibile per 2")#il print funziona solo con la f
	else: 
		print(f'non divisibile per 2 e 3')

#esercizio 7

	numeri=[1,2,3,4,5,6,7,8,9]
	numeri_len= len(numeri)
	somma=0
	for i in range(numeri_len):
		somma=somma+numeri[i]
	print(f"la somma dei numeri è {somma} ")
	media= somma/numeri_len
	print(f"la media è {media}")
	massimo=max(numeri)
	minimo=min(numeri)
	print(f"il massimo valore è {massimo}, mentre il minimo {minimo}")
#esercizio 8
	personal_data={"name":input("inserisci il tuo nome"),
				"surname":input("inserisci il tuo cognome"),
				"nascita":{"luogo":input("inserisci il luogo di nascita"),
							"giorno":input("inserisci il giorno")
								},
					"anni":input("inserisci la tua eta"),
					}
	nome=personal_data["name"]
	cognome=personal_data["surname"]
	eta=personal_data["anni"]
	print(f"ciao sono {nome} {cognome} e ho {eta}")
	print(f"\t{personal_data}:[nascita]")