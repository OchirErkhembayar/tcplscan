<?php
\printf('Hello, World!');
\printf('Updating this file');


switch ($i) {
    case 0:
        echo "$i is 0";
        break;
    case 1:
        echo "$i is 1";
        break;
}

if ($i = 0) {
    \printf("%s" $i);
} else {
    \printf("Else");
}

