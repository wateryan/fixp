import xml.etree.ElementTree
import os

ROOT_DIR = "spec"

def write_field(number, name, type):
    print number

def write_messages(mElements):
    print mElements.get("name")

def write_elements(elements):
    for m in elements.findall("messages/message"):
        write_messages(m)

def parse_file(file):
    elements = xml.etree.ElementTree.parse(ROOT_DIR + "/" + file).getroot()
    return elements

for filename in os.listdir(ROOT_DIR):
    e = parse_file(filename)
    write_elements(e)
