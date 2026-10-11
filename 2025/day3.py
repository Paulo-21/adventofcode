file = open("day3.input")
#file = open("day3.exemple")
res = 0
for line in file:
    maxi = 0
    next_m = 0
    l = list(line)
    #print(l[:-1])
    for k, n in enumerate(l[:-1]):
        n = int(n)
        if n > maxi and k < (len(l)-2):
            #print("m",n)
            maxi = n
            next_m = 0
        elif n > next_m:
            #print("mi",n)
            next_m = n
    #print("------------", maxi*10+next_m)
    res += maxi*10+next_m
print("Partie 1 : ",res)
file.seek(0)
res2 = 0
for line in file:
    line = line.rstrip('\r\n')
    nb_out = 0
    #print(line)

    l = list(line)
    window_size = len(l) - 12+1
    already_in = 0
    taken = [False] * len(l)
    #print("Const : ",len(l), " ", window_size)
    for k, c in enumerate(l):
        #print(already_in)
        if taken[k]:
            continue
        #print(l[k:(k+window_size-nb_out)])
        val_max = max(l[k:(k+window_size-nb_out)])
        idx_max = l[k:(k+window_size-nb_out)].index(val_max)
        #print(val_max, " ", idx_max)
        if 0 < idx_max:
            nb_out += 1
            if not taken[k+idx_max]:
                taken[k+idx_max] = True
                already_in += 1
        elif idx_max == 0:
            if not taken[k]:
                taken[k] = True
                already_in += 1
        if already_in == 12:
            #print("hhh")
            break
    caracteres_gardes = [l[i] for i in range(len(l)) if taken[i]]
    chaine_resultat = "".join(caracteres_gardes)
    #print(chaine_resultat)
    #print(len(chaine_resultat))
    res2 += int(chaine_resultat)
    #print(taken)
print("Partie 2 : ",res2)

if res2 == 3121910778619:
    print("GOOD")
