import os
import pandas as pd
import matplotlib.pyplot as plt

instance = 9
ants_threads = [(16, 16), (16, 1)]
legends = ["16 threads / 16 formigas / com elitismo", "1 thread / 16 formigas / sem elitismo"]
colors = ['#1f77b4', '#ff7f0e']

fig, ax = plt.subplots()
plotfile = f'sample_data/plots/i{instance}'

for legend, color, (ants, threads) in zip(legends, colors, ants_threads):
    infile = f'sample_data/"Indefinidas - i{instance}_durations_{ants}_ants_{threads}_threads.dat"'
    outfile = f'sample_data/tttplot-out/i{instance}_{ants}_ants_{threads}_threads'
    tttfile = outfile + '-ee.dat'

    print(infile)
    print(outfile)
    cmd = f'perl scripts/tttplots.pl -f {infile} -o {outfile}'
    print(cmd)
    os.system(cmd)

    data = pd.read_csv(tttfile, delim_whitespace=True, header=None)
    data.columns = ["Tempo (s)", "Probabilidade (p)"]

    print(data)
    print(data["Tempo (s)"], data["Probabilidade (p)"])

    data.plot(x="Tempo (s)", y="Probabilidade (p)", lw=0.3, ax=ax, label='', color=color)
    data.plot.scatter(x="Tempo (s)", y="Probabilidade (p)", ax=ax, label=legend, color=color)

plt.title(f'Instância {instance}')
plt.legend(loc='lower right')
plt.savefig(plotfile)
plt.show()
