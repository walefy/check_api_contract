# Check Api Contract

## Proposito

O proposito do check_api_contract é escrever um contrato e verificar se sua api segue aquele contrato. Atualmente os contratos são escritos em json mas penso em adicionar suporte a outros formatos de arquivos.

A ideia é que seja como um teste de integração só que vai funcionar independente da linguagem que você estiver utilizando.

## Stacks

- Rust -> Decidi utilizar rust porquê é uma linguagem compilada, veloz e que me diverte muito ^-^.
- Reqwest -> O reqwest é uma biblioteca para fazer requisições com rust, decidi utiliza-la para não implementar o conceito de async-await pois elá dá suporte a requisições bloqueantes com o modulo blocking.
- Serde -> O serde e serde_json está sendo utilizado para serializar e desserializar o arquivo de contrato, o serde foi escolhido pela sua utilização que é simples porem completa, além de suporte a vários tipos de arquivo.

## Como utilizar

O programa ainda está em construção então você ainda não conseguira utiliza-lo, mas quando esta parte estiver pronta o uso será da seguinte forma:

```bash
cac --contract ./example/simple.json
```
