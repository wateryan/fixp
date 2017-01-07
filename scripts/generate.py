import os
import xml.etree.ElementTree
from string import Template

ROOT_DIR = "spec"
E_FIELD = "field"
E_FIELDS = "fields/field"
E_MSG = "messages/message"
E_NAME = "name"
E_NUMBER = "number"

F_TRAIT = "trait Field {fn tag(&self) -> u8;}"
F_IMPL_TEMPLATE = Template(
    "impl Field for $fieldName {fn tag(&self) -> u8 {return $tagNumber;}}")
F_ENUM_TEMPLATE = Template("")
F_STRUCT_TEMPLATE = Template(
    "#[derive(Debug)]pub struct $fieldName {value: $type,}")

# TODO Figure out actual values for this, they're a best guess currently
TYPES = {
    "AMT": "f16",
    "CHAR": "char",
    "CURRENCY": "f16",
    "DATA": "u8[]",
    "DATE": "String",
    "DAYOFMONTH": "u8",
    "EXCHANGE": "String",
    "FLOAT": "f16",
    "INT": "u16",
    "LENGTH": "usize",
    "LOCALMKTDATE": "u64",
    "MONTHYEAR": "u8",
    "PRICE": "f16",
    "PRICEOFFSET": "i8",
    "QTY": "f16",
    "QUANTITY": "f64",
    "STRING": "String",
    "TIME": "u64",
    "UTCDATE": "String",
    "UTCTIMEONLY": "String",
    "UTCTIMESTAMP": "u64",
}


def write_elements(elements):
    fixv = "{}.{}.{}.{}".format(elements.get("type"),
                                elements.get("major"), elements.get("minor"),
                                elements.get("servicepack"))
    print fixv
    for field in elements.findall(E_FIELDS):
        print F_IMPL_TEMPLATE.substitute(fieldName=field.get(E_NAME), tagNumber=field.get("number"))
        if field.findall("value"):
            enumset = ",".join(v.get("description")
                               for v in field.findall("value"))
            print enumset
        else:
            fieldType = TYPES[field.get("type")]
            print F_STRUCT_TEMPLATE.substitute(fieldName=field.get(E_NAME), type=fieldType)
            # for value in field.findall("value"):

    # for msg in elements.findall(E_MSG):
    #     print msg.get(E_NAME)


def parse_file(file):
    elements = xml.etree.ElementTree.parse(ROOT_DIR + "/" + file).getroot()
    return elements

for filename in os.listdir(ROOT_DIR):
    e = parse_file(filename)
    write_elements(e)
