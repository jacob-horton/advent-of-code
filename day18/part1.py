def is_int(num):
  try:
    int(num)
    return True
  except ValueError:
    return False

class SnailNumber:
  def __init__(self, left, right, parent=None):
    self.left = left
    self.right = right
    self.parent = parent

  def get_depth(self):
    if self.parent == None:
      return 0
    else:
      return self.parent.get_depth() + 1

  def get_root(self):
    if self.parent == None:
      return self
    else:
      return self.parent.get_root()

  def get_left_to_right(self):
    result = []
    if type(self.left) is SnailNumber:
      result.extend(self.left.get_left_to_right())
    else:
      result.append(self.left)

    if type(self.right) is SnailNumber:
      result.extend(self.right.get_left_to_right())
    else:
      result.append(self.right)

    return result

  def magnitude(self):
    left = self.left
    right = self.right

    if type(self.left) is SnailNumber:
      left = self.left.magnitude()

    if type(self.right) is SnailNumber:
      right = self.right.magnitude()

    return 3 * left + 2 * right

  def add_left(self, value):
    if type(self.left) is SnailNumber:
      self.left.add_left(value)
    else:
      self.left += value

  def add_left_of(self, num, value):
    if self.left == num:
      if self.parent != None:
        self.parent.add_left_of(self, value)
    else:
      if type(self.left) is SnailNumber:
        self.left.add_right(value)
      else:
        self.left += value

  def add_right(self, value):
    if type(self.right) is SnailNumber:
      self.right.add_right(value)
    else:
      self.right += value

  def add_right_of(self, num, value):
    if self.right == num:
      if self.parent != None:
        self.parent.add_right_of(self, value)
    else:
      if type(self.right) is SnailNumber:
        self.right.add_left(value)
      else:
        self.right += value

  def explode_child(self, child):
    if self.left == child:
      self.left = 0

      self.add_left_of(self.left, child.left)
      if type(self.right) is SnailNumber:
        self.right.add_left(child.right)
      else:
        self.right += child.right
    elif self.right == child:
      self.right = 0

      if type(self.left) is SnailNumber:
        self.left.add_right(child.left)
      else:
        self.left += child.left
      self.add_right_of(self.right, child.right)
    else:
      raise RuntimeError('Child not recognised')
    

  def explode_and_split(self):
    # Loop through for first value to be > 10 or have a nesting of 4
    # Call explode_and_split again if this happens
    if self.get_depth() == 4:
      # Explode
      #print('explode', self)
      self.parent.explode_child(self)
      #print(self.get_root())
      self.get_root().explode_and_split()
      return True

    if type(self.left) is SnailNumber:
      did_explode_or_split = self.left.explode_and_split()
      if did_explode_or_split:
        return True
    else:
      if self.left >= 10:
        # Split
        #print('split')
        new_left = self.left // 2
        new_right = self.left - new_left
        self.left = SnailNumber(new_left, new_right, self)
        self.get_root().explode_and_split()
        return True

    if type(self.right) is SnailNumber:
      did_explode_or_split = self.right.explode_and_split()
      if did_explode_or_split:
        return True
    else:
      if self.right >= 10:
        # Split
        #print('split')
        new_left = self.right // 2
        new_right = self.right - new_left
        self.right = SnailNumber(new_left, new_right, self)
        self.get_root().explode_and_split()
        return True

    return False
  
  def __str__(self):
    return f'[{self.left},{self.right}]'

  @staticmethod
  def add(num1, num2):
    parent = SnailNumber(num1, num2)
    num1.parent = parent
    num2.parent = parent

    print('pre explode:', parent)
    parent.explode_and_split()
    return parent

  @classmethod
  def from_string(cls, string):
    depth = 0
    comma_index = -1

    for i, char in enumerate(string):
      if char == '[':
        depth += 1
      elif char == ']':
        depth -= 1
      elif char == ',':
        if depth == 1:
          comma_index = i
          break

    left_str = string[1:comma_index]
    right_str = string[comma_index+1:len(string)-1]

    if is_int(left_str):
      left = int(left_str)
    else:
      left = SnailNumber.from_string(left_str)

    if is_int(right_str):
      right = int(right_str)
    else:
      right = SnailNumber.from_string(right_str)

    result = cls(left, right)
    if type(left) is SnailNumber: left.parent = result
    if type(right) is SnailNumber: right.parent = result
    return result

prev = None
with open('test_input.txt', 'r') as f:
  line = f.readline()
  
  while line:
    line = line.strip()
    current = SnailNumber.from_string(line)
    if prev != None:
      print('adding: ', prev, '**', current)
      prev = SnailNumber.add(prev, current)
      print('result: ', prev)
      print()
    else:
      prev = current
    line = f.readline()

print(prev)