#!/usr/bin/env python3
from datetime import datetime

import dumps

dt = datetime(2025, 7, 10, 11, 51, 57, 960798)
print("Just datetime:")
result = dumps.json(dt)
print(result)

print("\nDatetime in dict:")
result2 = dumps.json({"dt": dt})
print(result2)
