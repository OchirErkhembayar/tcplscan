<?php
\printf('Hello, World!');
\printf('Updating this file');


switch ($i) {
    case 0:
        echo "$i is 0";
        if (true) {
            echo "If inside the switch statement";
        }
        break;
    case 1:
        echo "$i is 1";
        $var = match ($i) {
            1 => "Match statement inside a case of a switch",
            default => "Ending it.",
        };
        break;
    default:
        break;
}

if ($i = 0) {
    \printf("%s", $i);
} else {
    \printf("Else");
}

