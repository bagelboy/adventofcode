#!/usr/bin/env python3

import json

def sum_document(document):
    doc_type = type(document)
    if doc_type is int:
        return document
    if doc_type is list:
        return sum([sum_document(d) for d in document])
    if doc_type is dict:
        if 'red' in document.values():
            return 0
        return sum_document(list(document.values()))
    return 0

def run_tests():
    assert(sum_document(json.loads('[]')) == 0)
    assert(sum_document(json.loads('{}')) == 0)
    assert(sum_document(json.loads('[1,2,3]')) == 6)
    assert(sum_document(json.loads('{"a":2,"b":4}')) == 6)
    assert(sum_document(json.loads('[1,{"c":"red","b":2},3]')) == 4)
    assert(sum_document(json.loads('{"d":"red","e":[1,2,3,4],"f":5}')) == 0)
    assert(sum_document(json.loads('[1,"red",5]')) == 6)

def main():
    run_tests()
    with open('input') as json_data:
        print(sum_document(json.load(json_data)))

if __name__ == '__main__':
    main()
