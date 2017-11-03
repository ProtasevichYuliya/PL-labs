import re
sub_net_groups = dict()

def put_in_group(ip, sub_net):
    if sub_net in sub_net_groups:
        sub_net_groups[sub_net].append(ip)
    else:
        sub_net_group = list()
        sub_net_groups[sub_net] = sub_net_group
        sub_net_group.append(ip)

f = open('access.log', 'r')
IPs = re.findall(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}", f.read(), re.MULTILINE)
for ip in IPs:
    sub_net = re.search(r'^\d{1,3}\.\d{1,3}\.\d{1,3}\.', ip).group(0)
    put_in_group(ip, sub_net)

for sub_net_group in sub_net_groups:
    print(('{bold}'+sub_net_group + '0/24'+'{endcolor}').format(bold='\033[1m', endcolor='\033[0m'))
    for ip in sub_net_groups[sub_net_group]:
        print("- " + ip)
    print()