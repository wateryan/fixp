import os
import re
import xml.etree.ElementTree
from string import Template

from num2words import num2words

ROOT_DIR = "spec"

F_IMPORTS = "use field::Field;"
F_TRAIT = "trait Field {fn tag(&self) -> u16;}"
F_IMPL_TEMPLATE = Template(
    "impl Field for $fieldName {fn tag(&self) -> u16 {return $tagNumber;}}")
F_ENUM_TEMPLATE = Template(
    "#[derive(Debug)]pub enum $fieldName { $enumValues }")
F_STRUCT_TEMPLATE = Template(
    "pub struct $fieldName {value: $type,}")

M_IMPORTS_TEMPLATE = Template("use $fixVersion::fields::*;")
M_STRUCT_TEMPLATE = Template("struct $messageName { $structFields }")
M_F_REQ_TEMPLATE = Template("$fieldName: $fieldType,")
M_F_NREQ_TEMPLATE = Template("$fieldName: Option<$fieldType>,")

# TODO Figure out actual values for this, they're a best guess currently
TYPES = {
    "AMT": "f32",
    "BOOLEAN": "bool",
    "CHAR": "char",
    "COUNTRY": "String",
    "CURRENCY": "f32",
    "DATA": "[u8; 1024]",
    "DATE": "String",
    "DAYOFMONTH": "u8",
    "EXCHANGE": "String",
    "FLOAT": "f32",
    "INT": "u16",
    "LANGUAGE": "String",
    "LENGTH": "usize",
    "LOCALMKTDATE": "u64",
    "MONTHYEAR": "u8",
    "MULTIPLECHARVALUE": "String",
    "NUMINGROUP": "u16",
    "PERCENTAGE": "f32",
    "PRICE": "f32",
    "PRICEOFFSET": "i8",
    "QTY": "f32",
    "QUANTITY": "f64",
    "SEQNUM": "u64",
    "STRING": "String",
    "TIME": "u64",
    "TZTIMEONLY": "u64",
    "TZTIMESTAMP": "u64",
    "UTCDATE": "String",
    "UTCDATEONLY": "u16",
    "UTCTIMEONLY": "String",
    "UTCTIMESTAMP": "u64",
    "XMLDATA": "String",
}


def create_fields(elements):
    fields = F_IMPORTS + "\n"
    fixv = get_fix_version(elements)
    for field in elements.findall("fields/field"):
        fields += "\n" + F_IMPL_TEMPLATE.substitute(
            fieldName=field.get("name"), tagNumber=field.get("number"))
        if field.findall("value"):
            # Some of the spec has values such as 5yr which can't be valid
            # enums, so prepend _
            enumset = ",".join("_" + v.get("description").replace("_", "").title()
                               for v in field.findall("value"))
            fields += "\n" + F_ENUM_TEMPLATE.substitute(
                fieldName=field.get("name"), enumValues=enumset)
        else:
            field_type = TYPES[field.get("type")]
            fields += "\n" + F_STRUCT_TEMPLATE.substitute(
                fieldName=field.get("name"), type=field_type)

    file_dir = get_dir(fixv)
    if not os.path.exists(file_dir):
        os.makedirs(file_dir)
    fields_file = open(file_dir + "/fields.rs", "w")
    fields_file.write(fields)


def create_messages(elements):
    fixv = get_import(get_fix_version(elements))
    messages = M_IMPORTS_TEMPLATE.substitute(fixVersion=fixv) + "\n"
    for message in elements.findall("messages/message"):
        m_name = message.get("name") + "Message"
        m_fields = ""
        for field in message.findall("field"):
            field_type = field.get("name")
            field_name = to_snake(field_type[0].lower() + field_type[1:])
            if field.get("required") == 'Y':
                m_fields += M_F_REQ_TEMPLATE.substitute(
                    fieldName=field_name, fieldType=field_type)
            else:
                m_fields += M_F_NREQ_TEMPLATE.substitute(
                    fieldName=field_name, fieldType=field_type)
        messages += M_STRUCT_TEMPLATE.substitute(
            messageName=m_name, structFields=m_fields) + "\n"
    file_dir = get_dir(fixv)
    if not os.path.exists(file_dir):
        os.makedirs(file_dir)
    messages_file = open(file_dir + "/messages.rs", "w")
    messages_file.write(messages)


def parse_file(file):
    elements = xml.etree.ElementTree.parse(ROOT_DIR + "/" + file).getroot()
    return elements


def get_fix_version(elements):
    return "{}.{}.{}.{}".format(elements.get("type"),
                                elements.get("major"), elements.get("minor"),
                                elements.get("servicepack"))


def get_import(fix_version):
    return fix_version.lower().replace(".", "_")


def get_dir(fix_version):
    return "src/" + get_import(fix_version)


def to_snake(name):
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()


def export_mods(elements):
    fixv = get_fix_version(elements)
    file_dir = get_dir(fixv)
    mod_file = open(file_dir + "/mod.rs", "w")
    mod_file.write(
        "pub mod fields;\npub mod messages;"
    )

for filename in os.listdir(ROOT_DIR):
    print "Generating code for " + filename
    e = parse_file(filename)
    create_fields(e)
    create_messages(e)
    export_mods(e)
