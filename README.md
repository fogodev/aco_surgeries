# ACO Surgeries

Surgery scheduling using Ant Colony Optimization for the discipline of Metaheuristics in Combinatorial Optimization of master degree at PESC - UFRJ

## Instruções

Para instalar o compilador de Rust, seguir as instruções encontradas em:
https://www.rust-lang.org/pt-BR/tools/install

Após instalar, para compilar execute o seguinte comando no terminal:

`cargo build --release`

Com isso o binário compilado se encontrará no diretório `target/release`.

Basta então executar o programa como `./target/release/aco_surgeries --help` para maiores informações sobre flags e afins.

Implementamos vários argumentos para configurar e executar o programa sem a necessidade que o mesmo seja compilado repetidamente.

Como por exemplo: 

`./target/release/aco_surgeries -f "./sample_data/Indefinidas - i3.csv" -r 2`