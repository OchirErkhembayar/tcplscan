<?php

declare(strict_types=1);

namespace Foo\Baz;

use Foo\Types\MyType;
use Foob\NotAlias;

class Bar 
{
    readonly string $foo;

    static string $var;

    static $boo;
    
    private MyType $type;

    private static ReturnMe $foobary;

    public function __construct(
        private readonly string $bar,
        private readonly string $foobz,
    ) {}

    private function switcheroo($i): void
    {
        echo match ($i) {
            1 => "Foo!",
            2 => "Bar!",
            3 => "Baz!",
            default => "Boink!",
        };
        switch ($i) {
            case 1:
                for ($i = 0; $i < 420; $i++) {
                    echo "Ah! {$i}";
                }
                break;
            case 2:
                foreach ([1, 2, 3] as $num) {
                    printf("Argh!! %s", $num);
                }
                echo 2;
                break;
            case "wtf":
                switch ($i) {
                    case "omg":
                        echo "lol";
                        break;
                    case "holy moly":
                        match ($i) {
                            "dang" => "fooooo",
                            default => "enough pls",
                        };
                    default:
                        break;
                }
            default:
                echo $i;
                break;
        }

        $var = <<<'FOO'
            la la la
        FOO;
        $var = <<<FOO
            la la la
        FOO;
        $var::class;
    }
}
// Lol hi

