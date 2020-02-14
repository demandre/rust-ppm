#!/usr/bin/python3

import ppm
import argparse


class PPM():

    def run(self):
        args = self.getArgs()
        image = ppm.create_ppm(args['x'], args['y'])
        ppm.write_ppm("{}.ppm".format(args['name']), image)
        ppm.write_ppm("{}_grayscaled.ppm".format(args['name']), ppm.grayscale(image))
        ppm.write_ppm("{}_inverted.ppm".format(args['name']), ppm.invert(image))

    def getArgs(self):
        parser = argparse.ArgumentParser(
            description='Create an X by Y ppm image file, its grayscaled and inverted version'
        )
        parser.add_argument(
            '-x',
            help='Image width',
            required=True,
            type=int
        )
        parser.add_argument(
            '-y',
            help='Image height',
            required=True,
            type=int
        )
        parser.add_argument(
            '-n', '--name',
            help='Name of the file to create without file extension',
            required=True
        )
        return vars(parser.parse_args())


if __name__ == '__main__':
    o = PPM()
    o.run()

