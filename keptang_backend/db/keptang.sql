-- phpMyAdmin SQL Dump
-- version 5.2.0
-- https://www.phpmyadmin.net/
--
-- Host: 127.0.0.1
-- Generation Time: Apr 13, 2023 at 11:19 AM
-- Server version: 10.4.27-MariaDB
-- PHP Version: 8.2.0

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Database: `keptang`
--

-- --------------------------------------------------------

--
-- Table structure for table `moneylist`
--

CREATE TABLE `moneylist` (
  `list_id` int(11) NOT NULL,
  `description` text NOT NULL,
  `date` date NOT NULL,
  `amount` int(11) NOT NULL,
  `types` text NOT NULL,
  `user_id` int(11) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_general_ci;

--
-- Dumping data for table `moneylist`
--

INSERT INTO `moneylist` (`list_id`, `description`, `date`, `amount`, `types`, `user_id`) VALUES
(1, 'พ่อให้', '2023-04-12', 12000, 'income', 40955),
(3, 'สุ่มกาชา', '2023-04-12', 1000, 'expense', 40955),
(4, 'ขายโลมา', '2023-04-12', 100, 'income', 40956),
(5, 'ขายโลมา', '2023-04-12', 100, 'income', 40956),
(6, 'ขายโลมา', '2023-04-14', 2000, 'income', 40954),
(7, 'ทดสอบ', '2023-04-13', 1000, 'income', 40957),
(8, 'ขายโลมา', '2023-04-12', 100, 'income', 40956),
(9, 'รายรับทดสอบ', '2023-04-15', 1000, 'income', 40957),
(10, 'รายรับทดสอบ', '2023-04-13', 800, 'income', 40954),
(11, 'รายรับทดสอบ', '2023-04-13', 999, 'income', 40954);

-- --------------------------------------------------------

--
-- Table structure for table `userdata`
--

CREATE TABLE `userdata` (
  `user_id` int(11) NOT NULL,
  `user_name` text NOT NULL,
  `balance_total` int(11) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_general_ci;

--
-- Dumping data for table `userdata`
--

INSERT INTO `userdata` (`user_id`, `user_name`, `balance_total`) VALUES
(40954, 'chawanwit', 0),
(40955, 'nirut', 11000),
(40956, 'vivat', 15000),
(40957, 'sedthanan', 0);

--
-- Indexes for dumped tables
--

--
-- Indexes for table `moneylist`
--
ALTER TABLE `moneylist`
  ADD PRIMARY KEY (`list_id`);

--
-- Indexes for table `userdata`
--
ALTER TABLE `userdata`
  ADD PRIMARY KEY (`user_id`);

--
-- AUTO_INCREMENT for dumped tables
--

--
-- AUTO_INCREMENT for table `moneylist`
--
ALTER TABLE `moneylist`
  MODIFY `list_id` int(11) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=54;

--
-- AUTO_INCREMENT for table `userdata`
--
ALTER TABLE `userdata`
  MODIFY `user_id` int(11) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=40958;
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
