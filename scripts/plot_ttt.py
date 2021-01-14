import os
import pandas as pd
import matplotlib.pyplot as plt

for parallel in (False,): #(True, False):
    for instance in range(1, 2): # range(1, 15):
        par = str(parallel).lower()
        infile = f'sample_data/"Indefinidas - i{instance}_durations_par_{par}.dat"'
        outfile = f'sample_data/tttplot-out/{instance}_{par}'
        tttfile = outfile + '-ee.dat'
        plotfile = f'sample_data/plots/{instance}_{par}.png'

        print(infile)
        print(outfile)
        cmd = f'perl scripts/tttplots.pl -f {infile} -o {outfile}'
        print(cmd)
        os.system(cmd)

        data = pd.read_csv(tttfile, delim_whitespace=True, header=None)

        print(data)
        print(data[0], data[1])

        data.plot(x=0, y=1)
        plt.savefig(plotfile)
        plt.show()
