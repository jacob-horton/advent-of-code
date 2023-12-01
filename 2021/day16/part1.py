# VVVTTTAAAAAAAAA...
# V = version
# T = type ID

# Type 4 (literal):
# VVVTTTAAAAABBBBBCCCCC
# 110100101111111000101000
# Binary num padded with 0s until length multiple of 4
# Broken into groups of 4 bits, each group prefixed by a 1 except last, which has a 0
# 0s unlablelled are extra due to hex representation - ignore

# Other types (operators):
# Bit immediately after packet = length type ID
# 0 means 15 bits are the total length in bits of the sub-packets
# 1 means 11 bits are the number of sub-packets contained

def get_version_sum(packet, num_packets=1):
  original_len = len(packet)
  version_sum = 0

  while len(packet) > 0 and int(packet) != 0 and num_packets != 0:
    num_packets -= 1

    version = int(packet[:3], 2)
    type_id = int(packet[3:6], 2)

    if type_id == 4:
      i = 0
      while packet[6+i*5] == '1':
        i += 1
      
      version_sum += version
      packet = packet[6+(i+1)*5:]
    else:
      if packet[6] == '0':
        length = int(packet[7:22], 2)
        sub_packet = packet[22:22+length]

        sub_packet_sum, _ = get_version_sum(sub_packet, -1)
        version_sum += version + sub_packet_sum
        packet = packet[22+length:]
      else:
        num_sub_packets = int(packet[7:18], 2)
        sub_packet = packet[18:]

        sub_packet_sum, length = get_version_sum(sub_packet, num_sub_packets)
        version_sum += version + sub_packet_sum
        packet = packet[18+length:]

  return version_sum, original_len - len(packet)

with open('input.txt', 'r') as f:
  packet_hex = f.readline().strip()
  packet_dec = int(packet_hex, 16)
  packet_bin = bin(packet_dec).replace('0b', '')

  while len(packet_bin) % 4 != 0:
    packet_bin = '0' + packet_bin

print(get_version_sum(packet_bin)[0])

# 01100010000000001000000000000000000101100001000101010110001011001000100000000010000100011000111000110100
# 1100000000000001010100000000000000000001011000010001010110100010111000001000000000101111000110000010001101000000