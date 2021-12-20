area = (282,314,-80,-45)

def get_max_y(area):
  # If y_vel > 0, it will eventually fall back to y = 0
  # y_vel + 1 < -y_bottom

  for i in range(-area[2], 0, -1):
    y = 0
    y_vel = -i
    while y > area[3]:
      y += y_vel
      y_vel -= 1

    if y > area[2]:
      # Max y_vel, calc max y
      max_y = 0
      y_vel = i
      while y_vel > 0:
        max_y += y_vel
        y_vel -= 1

      return max_y

print(get_max_y(area))