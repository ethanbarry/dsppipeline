output = readtable("output.csv");

signal = timetable(table2array(output), 'SampleRate', 2048000);