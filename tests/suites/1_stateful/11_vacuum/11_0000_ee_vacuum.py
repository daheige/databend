#!/usr/bin/env python3

import os
import time
import mysql.connector
import sys
import signal
from multiprocessing import Process

CURDIR = os.path.dirname(os.path.realpath(__file__))
sys.path.insert(0, os.path.join(CURDIR, "../../../helpers"))

from native_client import NativeClient
from native_client import prompt

log = None

# client1 send long query, client mydb fetch the long query and kill it.
# Use to mock in MySQL Client press Ctrl C to intercept a long query.

mydb = mysql.connector.connect(
    host="127.0.0.1", user="root", passwd="root", port="3307"
)

def insert_data(name):
    mycursor = mydb.cursor()
    value = 1
    while value < 20:
        sql = "insert into table gc_test values(%d);" % value
        mycursor.execute(sql)
        value +=1

def compact_data(name):
    mycursor = mydb.cursor()
    mycursor.execute('optimize table gc_test all;')

if __name__ == '__main__':
    with NativeClient(name="client1>") as client1:
        client1.expect(prompt)

        client1.send("drop table if exists gc_test;")
        client1.expect(prompt)

        client1.send("create table gc_test(a int);");
        client1.expect(prompt)

        insert_data('insert_data')

        compact_data('compact_data')

        mycursor = mydb.cursor()
        mycursor.execute('select a from gc_test order by a;')
        old_datas = mycursor.fetchall()

        client1.send("vacuum table gc_test retain 0 hours;");
        client1.expect(prompt)

        mycursor.execute('select a from gc_test order by a;')
        datas = mycursor.fetchall()

        if old_datas != datas:
            print("vacuum lose data: %s : %s" % (old_datas, datas))
        else:
            print("vacuum success")


