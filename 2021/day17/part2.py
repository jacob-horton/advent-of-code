area = (282,314,-80,-45)
#area = (20,30,-10,-5)

def does_hit_target(area, x_vel, y_vel):
  x = 0
  y = 0

  while x < area[1] and y > area[2]:
    x += x_vel
    y += y_vel

    if x_vel > 0: x_vel -= 1
    elif x_vel < 0: x_vel += 1
    y_vel -= 1

    if x <= area[1] and y >= area[2] and x >= area[0] and y <= area[3]:
      return True

  return x <= area[1] and y >= area[2] and x >= area[0] and y <= area[3]

def get_possible_velocities(area):
  # If y_vel > 0, it will eventually fall back to y = 0
  # y_vel + 1 < -y_bottom

  possible_vels = []
  for y_vel in range(area[2]-1, 1-area[2]):
    for x_vel in range(area[1]+1):
      if does_hit_target(area, x_vel, y_vel):
        #print(x_vel, y_vel)
        possible_vels.append((x_vel, y_vel))

  return possible_vels

print(len(get_possible_velocities(area)))