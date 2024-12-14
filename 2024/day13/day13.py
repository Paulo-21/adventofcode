import re
#from gurobipy import Model, GRB
import numpy as np
def solve(a1, b1, a2, b2, resx, resy):
    A = np.array([[a1, b1],
                  [a2, b2]], dtype=int)
    B = np.array([resx, resy], dtype=int)
    
    # Résoudre A * X = B
    try:
        print(a1,"+", b1 ,"==", resx)
        print(a2,"+", b2, "==", resy)
        solution = np.linalg.solve(A, B)
        #print(f"A {solution[0]} B {solution[1]}")
        if a1 * round(solution[0]) + round(solution[1])*b1 == resx and a2 * round(solution[0]) + round(solution[1])*b2 == resy:
            return 3 * int(solution[0]) + int(solution[1])
        else:
            return 0
    except np.linalg.LinAlgError:
        # Système singulier ou pas de solution
        return 0
    """
    # Créer un modèle Gurobi
    model = Model("Solver")
    #model.Params.DualReductions = 0
    model.setParam("OutputFlag", 0)  # Désactiver les logs de Gurobi

    # Définir les variables entières
    x = model.addVar(vtype=GRB.INTEGER, name="x", lb=0, ub=2**64)
    y = model.addVar(vtype=GRB.INTEGER, name="y", lb=0, ub=2**64)
    model.update()
    # Définir l'objectif : minimiser une expression
    model.setObjective( (3*x) + y, GRB.MINIMIZE)
    print(a1,"+", b1 ,"==", resx)
    print(a2,"+", b2, "==", resy)
    # Ajouter les contraintes
    #print(a1 * x + b1 * y == resx)
    #print(a2 * x + b2 * y == resy)
    model.addConstr(a1 * x + b1 * y == resx, "Constraint1")
    model.addConstr(a2 * x + b2 * y == resy, "Constraint2")
    
    # Résoudre le problème
    model.optimize()
    if model.status == GRB.OPTIMAL:
        # Obtenir les valeurs des variables
        x_value = x.X
        y_value = y.X
        print("YES")
        return 3 * x_value + y_value
    else:
        print("NON", model.status)
        return 0
    """

def main():
    #with open("exemple", "r", encoding="utf-8") as f:
    with open("input", "r", encoding="utf-8") as f:
        input_data = f.read()

    reB = re.compile(r"X\+(\d+), Y\+(\d+)")
    reP = re.compile(r"X=(\d+), Y=(\d+)")

    a1 = a2 = b1 = b2 = 0
    res1 = 0

    for line in input_data.splitlines():
        if not line.strip():
            continue
        s = line.split(":", 1)
        if len(s) < 2:
            continue
        key, val = s[0].strip(), s[1].strip()

        if key.startswith("Button A"):
            caps = reB.search(val)
            if caps:
                a1 = int(caps.group(1))
                a2 = int(caps.group(2))
        elif key.startswith("Button B"):
            caps = reB.search(val)
            if caps:
                b1 = int(caps.group(1))
                b2 = int(caps.group(2))
                #print("B : ",b1, " ", b2 )
        elif key.startswith("Prize"):
            caps = reP.search(val)
            if caps:
                #resx = int(caps.group(1))
                resx = int(caps.group(1)) + 10000000000000
                #resy = int(caps.group(2))
                resy = int(caps.group(2)) + 10000000000000
                r = solve(a1, b1, a2, b2, resx, resy)
                res1 += r
    print(int(res1))

if __name__ == "__main__":
    main()
"""
total = 0
for block in open("input").read().split("\n\n"):
    ax, ay, bx, by, px, py = map(int, re.findall(r"\d+", block))
    px += 10000000000000
    py += 10000000000000
    ca = (px * by - py * bx) / (ax * by - ay * bx)
    cb = (px - ax * ca) / bx
    if ca % 1 == cb % 1 == 0:
        total += int(ca * 3 + cb)

print(total)
"""