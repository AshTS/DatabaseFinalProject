import requests

response = requests.get("https://docs.google.com/spreadsheets/d/1YfCcfszmOJ5lYL_rUz-mIcBKqo0Fd4Lhpy4k3nf4-YA/gviz/tq?tqx=out:csv&sheet=spells-table.csv")
output = open("original.csv", 'w')
output.write(response.text)
output.close()