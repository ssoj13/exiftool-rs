#!/usr/bin/env perl
# ExifTool tag table dumper for Rust codegen
# Outputs JSON with all tag definitions including nested tables

use strict;
use warnings;
use JSON::PP;
use File::Basename;
use File::Spec;

# Add ExifTool lib to path
my $script_dir = dirname(__FILE__);
my $exiftool_lib = File::Spec->catdir($script_dir, '..', '_ref', 'exiftool', 'lib');
unshift @INC, $exiftool_lib;

# Load ExifTool modules
require Image::ExifTool;
require Image::ExifTool::Exif;

# Track visited tables to avoid infinite recursion
my %visited_tables;

# All tables we find
my %all_tables;

# Vendors to extract
my @vendors = qw(Canon Nikon FujiFilm Sony Olympus Panasonic Pentax Samsung Apple);

my %output;

# Extract tag tables for each vendor (starting from Main)
for my $vendor (@vendors) {
    my $module = "Image::ExifTool::$vendor";
    eval "require $module";
    if ($@) {
        warn "Failed to load $module: $@\n";
        next;
    }
    
    %visited_tables = ();
    my $main_table = "Image::ExifTool::${vendor}::Main";
    collect_all_tables($main_table);
    
    # Copy collected tables for this vendor
    $output{$vendor} = { %all_tables };
    %all_tables = ();
}

# Also extract base EXIF tags
%visited_tables = ();
collect_all_tables("Image::ExifTool::Exif::Main");
$output{Exif} = { %all_tables };
%all_tables = ();

# GPS tags
%visited_tables = ();
require Image::ExifTool::GPS if eval { require Image::ExifTool::GPS; 1 };
collect_all_tables("Image::ExifTool::GPS::Main");
$output{GPS} = { %all_tables };
%all_tables = ();

# Output JSON
my $json = JSON::PP->new->pretty->canonical;
print $json->encode(\%output);

sub collect_all_tables {
    my ($table_name) = @_;
    
    return if $visited_tables{$table_name}++;
    
    # Try to load the module
    if ($table_name =~ /^Image::ExifTool::(\w+)::/) {
        my $mod = "Image::ExifTool::$1";
        eval "require $mod" unless $mod eq 'Image::ExifTool';
    }
    
    no strict 'refs';
    my $table = \%{$table_name};
    use strict 'refs';
    
    return unless %$table;
    
    my %table_info;
    $table_info{format} = $table->{FORMAT} if $table->{FORMAT};
    $table_info{first_entry} = $table->{FIRST_ENTRY} if defined $table->{FIRST_ENTRY};
    
    # Extract tags
    my %tags;
    for my $id (keys %$table) {
        next unless $id =~ /^-?\d+$/;  # Only numeric IDs
        
        my $entry = $table->{$id};
        next unless ref $entry;
        
        my $tag_info = extract_tag_info($entry);
        if ($tag_info) {
            $tags{$id} = $tag_info;
            
            # Recursively collect sub-tables
            if ($tag_info->{sub_table}) {
                collect_all_tables($tag_info->{sub_table});
            }
        }
    }
    
    $table_info{tags} = \%tags if %tags;
    
    # Store with short name
    my $short_name = $table_name;
    $short_name =~ s/^Image::ExifTool:://;
    $all_tables{$short_name} = \%table_info;
}

sub extract_tag_info {
    my ($entry) = @_;
    
    # Handle array (conditional tags) - take first
    if (ref $entry eq 'ARRAY') {
        $entry = $entry->[0];
    }
    
    return undef unless ref $entry eq 'HASH';
    
    my %info;
    
    # Name (required)
    $info{name} = $entry->{Name} if $entry->{Name};
    return undef unless $info{name};
    
    # Data type
    $info{format} = $entry->{Writable} if $entry->{Writable};
    $info{format} //= $entry->{Format} if $entry->{Format};
    
    # Value mappings from PrintConv (all values as strings)
    if ($entry->{PrintConv} && ref $entry->{PrintConv} eq 'HASH') {
        my %values;
        for my $k (keys %{$entry->{PrintConv}}) {
            next unless defined $k && $k =~ /^-?\d+$/;
            my $v = $entry->{PrintConv}{$k};
            # Ensure value is string
            $values{$k} = ref $v ? '' : "$v";
        }
        $info{values} = \%values if %values;
    }
    
    # SubDirectory reference
    if ($entry->{SubDirectory} && $entry->{SubDirectory}{TagTable}) {
        $info{sub_table} = $entry->{SubDirectory}{TagTable};
    }
    
    # Groups
    if ($entry->{Groups}) {
        $info{group} = $entry->{Groups}{2} if $entry->{Groups}{2};
    }
    
    # Count not needed for codegen

    return \%info;
}
