price = 12.50
quantity = 3

total = price * quantity
print("Order total:", total)

price = "12.50"

print("Price type:", type(price).__name__)

try:
    result = price + quantity
    print(result)
except TypeError as error:
    print("Python error:", error)
