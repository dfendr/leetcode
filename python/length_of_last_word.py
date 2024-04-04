s = "hello world"
s = "   fly me   to   the moon  "
for word in s.split():
    print(word)

print(s.split()[:-1][])
# len(s.split()[:-1])
# print(len(s.split()[:-1][0]))
