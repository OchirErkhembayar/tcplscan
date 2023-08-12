<?php

declare(strict_types=1);

namespace Foo\Baz;

class Bar 
{
    public function __construct(
        private readonly string $bar,
        private readonly string $foobz,
    ) {}

    public function baz(): void
    {
        if ($lol) {
            echo "foo!";
        } elseif (1) {
            echo "Bar!";
        } elseif (2) {
            echo "Two!";
        } else {
            echo "Else!";
        }
        echo "\n";
    }

    function noReturn() {}

    /**
     * @return string
     * @throws \RuntimeException
     */
    private function zab(string $abz, array $baris): string
    {
        throw \RuntimeException("I love hard to debug code!");
        try {
            throw new \Exception("Don't throw exceptions, kids.");
        } catch (\Exception $exception) {
            echo "Very bad."
        }
        for ($i = 0; $i < 5; $i++) {
            echo $;
        }
        foreach ($baris as $bari) {
            \var_dump($bari);
        }
        return $abz;
    }

    private function switcheroo($i): void
    {
        match ($i) {
            1 => echo "Foo!",
            2 => echo "Bar!",
            3 => echo "Baz!",
            default => echo "Boink!",
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
    }
}
// Lol hi

